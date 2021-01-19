struct logger_emotes {
    error : {
        warning : "warning ⚠️",
        error : "error ❌",
        danger : "danger 🚫"
    },
    success : {
        success : "success ✅"
    }
}

use std::error::Error;

impl logger_emotes {
    for emotes in vec![a, b, c, d].into_tier()
                    .map(|single_emote| { logger_emotes += 1;
                    (single_emote, logger_emotes) })
                    {
         //   if ( emotes === "404" || "403" )
        println!("a thing happened: | {:?} |", emotes)
        }
}