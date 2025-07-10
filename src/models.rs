use serde::{Serialize, Deserialize};
use chrono::{ DateTime,Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMeta{
    pub id:String,
    pub filename:String,
    pub saved_path:String,
    pub upload_time:DateTime<Utc>,
}


