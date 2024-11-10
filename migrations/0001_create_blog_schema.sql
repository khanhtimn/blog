-- Create categories table
CREATE TABLE categories (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT
);

CREATE INDEX categories_slug_idx ON categories(slug);

-- Create blog_posts table
CREATE TABLE blog_posts (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    hero_image VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    published_at TIMESTAMP WITH TIME ZONE NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE
);

CREATE INDEX blog_posts_slug_idx ON blog_posts(slug);
CREATE INDEX blog_posts_published_at_idx ON blog_posts(published_at DESC);

-- Create junction table for blog_posts and categories
CREATE TABLE blog_posts_categories (
    blog_post_id BIGINT REFERENCES blog_posts(id) ON DELETE CASCADE,
    category_id BIGINT REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (blog_post_id, category_id)
);

CREATE INDEX blog_posts_categories_post_idx ON blog_posts_categories(blog_post_id);
CREATE INDEX blog_posts_categories_category_idx ON blog_posts_categories(category_id);
