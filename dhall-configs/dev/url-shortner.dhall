let redis_cfg = {
    host = "0.0.0.0",
    port = 30001,
    cluster_enabled = True,
    cluster_urls = [""],
    use_legacy_version = False,
    pool_size = 50,
    reconnect_max_attempts = 10,
    reconnect_delay = 5000,
    default_ttl = 3600,
    default_hash_ttl = 3600,
    stream_read_count = 100,
    partition = 0,
}

in {
    port = 9023,
    workers = 1,
    redis_cfg = redis_cfg,
    redis_expiry = 86400,
    request_timeout = 9000,
    internal_auth_api_key = "some-internal-api-key",
    short_code_length = 6,
    shortened_base_url = "http://localhost:9023",
    max_retries_for_shortening = 5,
}