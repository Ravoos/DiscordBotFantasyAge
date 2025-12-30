use serenity::all::CreateCommandOption;
use serenity::all::CreateCommand;
use serenity::all::CommandOptionType;

pub fn register_main_roll_command() -> CreateCommand {
    CreateCommand::new("mainroll")
        .description("Roll 3d6 and add a modifier")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "modifier",
                "Modifier to add to the roll",
            )
            .required(false),
        )
}

pub fn register_damage_roll_command() -> CreateCommand {
    CreateCommand::new("damageroll")
        .description("Roll Xd6 + Y damage")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "number_of_dice",
                "Amounts of six sided dice to roll (Xd6)",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "damage_modifier",
                "Additional damage modifier",
            )
            .required(false),
        )
}

pub fn register_basic_stunts_command() -> CreateCommand {
    CreateCommand::new("basicstunts")
        .description("Get a list of the basic stunts available in Fantasy AGE")
        .add_option(
        CreateCommandOption::new(
            CommandOptionType::String,
                "type",
                "Type of basic stunts: combat, social, exploration, spell",
           )
        .required(true),
        )
}

pub fn register_class_stunts_command() -> CreateCommand {
    CreateCommand::new("classstunts")
        .description("Get a list of stunts for a specific Fantasy AGE class")
        .add_option(
            CreateCommandOption::new(
            CommandOptionType::String,
                "class",
                "Class name: warrior, rogue, mage, envoy",
            )
        .required(true),
        )
}