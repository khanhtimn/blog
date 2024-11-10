INSERT INTO categories (name, slug, description)
VALUES ('Programming', 'programming', 'Technical articles about programming'),
       ('Design', 'design', 'Articles about design principles and practices'),
       ('Tutorial', 'tutorial', 'Step-by-step guides and tutorials');

-- First, insert some blog posts
INSERT INTO blog_posts (title,
                        description,
                        hero_image,
                        content,
                        published_at,
                        slug)
VALUES ('Getting Started with PostgreSQL',
        'A comprehensive guide to setting up and using PostgreSQL for beginners',
        '/images/postgres-hero.jpg',
        E'# Introduction to PostgreSQL\n\nPostgreSQL is a powerful open-source database system that has earned a strong reputation for its reliability, feature robustness, and performance.\n\n## Getting Started\n\nFirst, you''ll need to install PostgreSQL on your system...',
        '2024-01-15 09:00:00+00',
        'getting-started-with-postgresql'),
       ('Design Patterns in Modern Web Development',
        'Exploring essential design patterns for building scalable web applications',
        '/images/design-patterns-hero.jpg',
        E'# Design Patterns in Web Development\n\nDesign patterns are reusable solutions to common problems in software design. Let''s explore some of the most useful patterns...',
        '2024-02-01 10:30:00+00',
        'design-patterns-modern-web-development'),
       ('Building Your First REST API',
        'Step-by-step tutorial on creating a REST API from scratch',
        '/images/api-hero.jpg',
        E'# Creating a REST API\n\nIn this tutorial, we''ll walk through the process of building a REST API using modern best practices...',
        '2024-03-01 14:15:00+00',
        'building-first-rest-api');

-- Now, link the blog posts to their respective categories
INSERT INTO blog_posts_categories (blog_post_id, category_id)
SELECT blog_posts.id,
       categories.id
FROM blog_posts,
     categories
WHERE blog_posts.slug = 'getting-started-with-postgresql'
  AND categories.slug IN ('programming', 'tutorial');

INSERT INTO blog_posts_categories (blog_post_id, category_id)
SELECT blog_posts.id,
       categories.id
FROM blog_posts,
     categories
WHERE blog_posts.slug = 'design-patterns-modern-web-development'
  AND categories.slug IN ('programming', 'design');

INSERT INTO blog_posts_categories (blog_post_id, category_id)
SELECT blog_posts.id,
       categories.id
FROM blog_posts,
     categories
WHERE blog_posts.slug = 'building-first-rest-api'
  AND categories.slug IN ('programming', 'tutorial');
