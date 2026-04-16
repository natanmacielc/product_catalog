CREATE TABLE IF NOT EXISTS products (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    brand VARCHAR(255) NOT NULL,
    category VARCHAR(255) NOT NULL,
    price_cents BIGINT NOT NULL CHECK (price_cents >= 0),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_products_name ON products (LOWER(name));
CREATE INDEX IF NOT EXISTS idx_products_brand ON products (LOWER(brand));
CREATE INDEX IF NOT EXISTS idx_products_category ON products (LOWER(category));

INSERT INTO products (name, brand, category, price_cents)
VALUES
    ('Notebook Gamer X', 'Dell', 'Informatica', 450000),
    ('Mouse Pro', 'Logitech', 'Perifericos', 15000),
    ('Teclado Mecânico K8', 'Keychron', 'Perifericos', 35000),
    ('Monitor UltraWide 34', 'LG', 'Monitores', 189900);