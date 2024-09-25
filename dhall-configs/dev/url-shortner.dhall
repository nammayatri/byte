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

let LogLevel = < TRACE | DEBUG | INFO | WARN | ERROR | OFF >

let logger_cfg = {
    level = LogLevel.INFO,
    log_to_file = False
}

let expired_short_code_fallback_url_map = {
    mtb = "https://nammayatri.in/mtb",
    rtk = "https://nammayatri.in/rtk",
}

in {
    port = 9023,
    workers = 1,
    logger_cfg = logger_cfg,
    redis_cfg = redis_cfg,
    redis_expiry = 86400,
    request_timeout = 9000,
    log_unprocessible_req_body = ["UNPROCESSIBLE_REQUEST", "REQUEST_TIMEOUT", "LARGE_PAYLOAD_SIZE"],
    max_allowed_req_size = 512000, -- 500 KB
    internal_auth_api_key = "some-internal-api-key",
    short_code_length = 6,
    shortened_base_url = "http://localhost:9023",
    max_retries_for_shortening = 5,
    default_fallback_url = "https://nammayatri.in",
    expired_short_code_fallback_url_map,
}