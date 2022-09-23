use serde::de::DeserializeOwned;

pub struct InputPool {
    pub pool: std::collections::HashMap<String, serde_json::Value>
}

impl InputPool {
    pub fn new() -> InputPool {
        InputPool {
            pool: std::collections::HashMap::<String, serde_json::Value>::new()
        }
    }
    pub fn contains(&self, key: &str) -> bool {
        self.pool.contains_key(key)
    }
    pub fn insert(&mut self, key: &str, value: serde_json::Value) {
        self.pool.insert(key.to_string(), value);
    }
    pub fn get<T>(&self, key: &str) -> Result<T, Box<dyn std::error::Error>> 
        where T: DeserializeOwned
    {
        let val: &serde_json::Value =  self.pool.get(key).ok_or("Value not stored in inputs")?;
        let obj: T = serde_json::from_value(val.clone())?;
        Ok(obj)
    }
    pub fn get_string(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let val_res = self.get::<String>(key)?;
        Ok(val_res)
    }
    pub fn get_u64(&self, key: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let val_res = self.get::<u64>(key)?;
        Ok(val_res)
    }
    pub fn get_f64(&self, key: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let val_res = self.get::<f64>(key)?;
        Ok(val_res)
    }
    pub fn get_i64(&self, key: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let val_res = self.get::<i64>(key)?;
        Ok(val_res)
    }
}

impl Default for InputPool {
    fn default() -> Self {
        InputPool::new()
    }
}

