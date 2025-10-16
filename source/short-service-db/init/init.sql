
CREATE TABLE IF NOT EXISTS tb_link (
    tb_link_id       BIGSERIAL PRIMARY KEY,
    tb_link_short    TEXT NOT NULL,
    tb_link_long     TEXT NOT NULL,
    tb_link_creation TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tb_link_expires  TIMESTAMP,
    tb_link_count    BIGINT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS tb_link_short_index ON tb_link (tb_link_short);
CREATE INDEX IF NOT EXISTS tb_link_expires_index ON tb_link (tb_link_expires);

-- dev data
INSERT INTO tb_link (
    tb_link_short,
    tb_link_long
) VALUES (
  'rust',
  'https://rust-lang.org/'
);

INSERT INTO tb_link (
    tb_link_short,
    tb_link_long
) VALUES (
  'google',
  'https://google.com/'
);
