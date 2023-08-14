/// Get the Lua script for computing the size of queue.
///
/// - `KEYS[1]` - The key the lock was taken with
/// - `ARGV[1]` - The owner of the lock
pub const RELEASE: &str = r#"
if redis.call("get",KEYS[1]) == ARGV[1] then
    return redis.call("del",KEYS[1])
else
    return 0
end
"#;
