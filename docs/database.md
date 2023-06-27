## user 表

id（主键）, create_time, username（唯一约束）, password, sex, update_time, age

## history 表

id（主键）, user_id, news_id, last_view_time

## interest 表

id（主键）, user_id, news_tag, weight, last_view_time

## news 表

id（主键）, create_time, title, content, likes

## news_tag 表

id（主键）, tag_name, news_id