use rbatis::snowflake::Snowflake;

// Fri, 01 Jan 2021 00:00:00 GMT
const ITCHAT_EPOCH: i64 = 1609459200000;

lazy_static! {
    static ref SNOWFLAKE: Snowflake = {
        let mut snowflake = Snowflake::default();
        snowflake.set_epoch(ITCHAT_EPOCH);
        snowflake
    };
}

pub fn generate() -> u64 {
    SNOWFLAKE.generate() as u64
}
