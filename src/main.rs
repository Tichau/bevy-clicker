use bevy::{
    prelude::*,
    reflect::TypeUuid,
    utils::Duration,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DataPlugin)
        .add_plugin(GamePlugin)
        .run();
}

// Data

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        info!("Build DataPlugin...");
        app.add_asset::<RecipeDefinition>()
            .add_startup_system(DataPlugin::initialize)
            .add_startup_system(DataPlugin::load_recipes);
    }
}

impl DataPlugin {
    fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Tell the asset server to watch for asset changes on disk:
        asset_server.watch_for_changes().unwrap();
    }

    fn load_recipes(mut recipes: ResMut<Assets<RecipeDefinition>>) {
        info!("Load recipes...");
        recipes.add(RecipeDefinition {
            inputs: Vec::new(),
            outputs: vec![ ResourceAmount{ resource: Resource::CopperOre, amount: 1 } ],
            craft_duration: Duration::from_millis(20833),
        });
        recipes.add(RecipeDefinition {
            inputs: Vec::new(),
            outputs: vec![ ResourceAmount{ resource: Resource::IronOre, amount: 1 } ],
            craft_duration: Duration::from_millis(20833),
        });
    }
}

#[derive(Debug)]
enum Resource {
    CopperOre,
    IronOre,
}

#[derive(Debug)]
struct ResourceAmount {
    resource: Resource,
    amount: u64,
}

#[derive(Debug, TypeUuid)]
#[uuid = "820df7c7-e897-420e-b85b-93ad0ca824a0"]
struct RecipeDefinition {
    inputs: Vec<ResourceAmount>,
    outputs: Vec<ResourceAmount>,
    craft_duration: Duration,
}

// Game

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        info!("Build GamePlugin...");
        app.insert_resource(TickTimer(Timer::from_seconds(1.0, true)))
            .add_startup_system(initialize)
            .add_system(tick_player_craft_queue);
    }
}

struct TickTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct CraftQueue {
    items: Vec<CraftTask>,
}

#[derive(Component)]
struct PlayerResource {
    id: Resource,
    amount: u64,
}

impl PlayerResource {
    fn new(id: Resource) -> Self {
        Self {
            id,
            amount: 0,
        }
    }
}

#[derive(Component)]
struct CraftTask {
    recipe: Handle<RecipeDefinition>,
    time_spent: Duration,
}

impl CraftTask {
    fn new(recipe: Handle<RecipeDefinition>) -> Self {
        Self {
            recipe,
            time_spent: Duration::from_millis(0),
        }
    }
}

fn initialize(mut commands: Commands, recipes: Res<Assets<RecipeDefinition>>) {
    let recipe_handle = recipes.get_handle(recipes.iter().next().unwrap().0);

    commands.spawn()
        .insert(Player)
        .insert(CraftQueue{ items: vec![ CraftTask::new(recipe_handle) ] })
        .insert(PlayerResource::new(Resource::CopperOre))
        .insert(PlayerResource::new(Resource::IronOre));
}

fn tick_player_craft_queue(time: Res<Time>, mut timer: ResMut<TickTimer>, mut query: Query<&mut CraftQueue, With<Player>>, recipes: Res<Assets<RecipeDefinition>>) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut worked_duration = timer.0.duration();
        let (mut queue) = query.single_mut();
        while queue.items.len() > 0 && worked_duration > Duration::from_millis(0) {
            let craft = queue.items.get_mut(0).unwrap();
            let recipe = recipes.get(&craft.recipe).unwrap();
            let remaining_work = recipe.craft_duration - craft.time_spent;
            let time_to_spend = if worked_duration < remaining_work { worked_duration } else { remaining_work };
            craft.time_spent += time_to_spend;
            worked_duration -= time_to_spend;
            info!("Craft time spent: {}ms", craft.time_spent.as_millis());

            if craft.time_spent >= recipe.craft_duration {
                info!("Craft terminated");
                // TODO: Credit resources
                queue.items.remove(0);
            }

        }
    }
}
