CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE url_mappings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    short_code VARCHAR(10) UNIQUE NOT NULL,
    original_url TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    expires_at TIMESTAMP WITH TIME ZONE,
    click_count INTEGER DEFAULT 0
);

CREATE INDEX idx_short_code ON url_mappings (short_code);