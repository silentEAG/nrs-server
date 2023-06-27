CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  create_time TIMESTAMP NOT null DEFAULT now(),
  username VARCHAR(255) UNIQUE NOT NULL,
  password VARCHAR(255) NOT NULL,
  sex VARCHAR(10) NOT NULL,
  update_time TIMESTAMP NOT null DEFAULT now(),
  age INTEGER NOT NULL
);

CREATE TABLE history (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  news_id INTEGER NOT NULL,
  last_view_time TIMESTAMP NOT NULL
);

CREATE TABLE interest (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  news_tag VARCHAR(255) NOT NULL,
  weight INTEGER NOT NULL,
  last_view_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE news (
  id SERIAL PRIMARY KEY,
  create_time TIMESTAMP NOT null DEFAULT now(),
  title VARCHAR(255) NOT NULL,
  source VARCHAR(255) NOT NULL,
  abstracts TEXT NOT NULL,
  content TEXT NOT NULL,
  likes INTEGER NOT NULL
);

CREATE TABLE news_tag (
  id SERIAL PRIMARY KEY,
  tag_name VARCHAR(255) NOT NULL,
  news_id INTEGER NOT NULL REFERENCES news(id)
);


-- index
CREATE INDEX idx_history_user_id ON history(user_id);
CREATE INDEX idx_history_news_id ON history(news_id);
CREATE INDEX idx_interest_user_id ON interest(user_id);
CREATE INDEX idx_interest_news_tag ON interest(news_tag);
CREATE INDEX idx_news_title ON news(title);
CREATE INDEX idx_news_likes ON news(likes);
CREATE INDEX idx_news_tag_news_id ON news_tag(news_id);


-- fix
ALTER TABLE interest ADD CONSTRAINT interest_user_id_news_tag_key UNIQUE (user_id, news_tag);


-- SELECT news.id, news.title, news.abstracts, news.source, news.create_time, news.likes, news_tag.tag_name
-- FROM history, news
-- JOIN news_tag ON news_tag.news_id = news.id
-- WHERE  history.user_id = $1 AND history.news_id = news.id
-- GROUP BY news.id