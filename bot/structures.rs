pub struct config {
    let bot;
    let user;

    defaultDataValues[user]     :      0,
    maxValuesPerVariable[user]  :      300,

    maxVarsPerBot[bot]          :      100,
    defaultPresence[bot]        :      ("windows"),

    maxStatusPerUser[user]      :      10,

    options[user, bot]          :      {
        compress?: true,
        allow_external_apis?: true,
        use_npm?: true,
        javascript?: true,

        allowReactions?: true,

        allow_bcfd?: true,
        allow_bdfd?: false,
    },
}