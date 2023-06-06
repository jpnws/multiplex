CREATE TABLE IF NOT EXISTS threads (thread_id SERIAL PRIMARY KEY,
                                                             title VARCHAR(255) NOT NULL,
                                                                                user_id INT NOT NULL,
                                                                                            creation_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                     modified_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                              user_ip VARCHAR(255) NOT NULL);


CREATE OR REPLACE FUNCTION update_modified_column() RETURNS TRIGGER AS $$
BEGIN
   NEW.modified_date = NOW();
   RETURN NEW;
END;
$$ language 'plpgsql';


CREATE TRIGGER update_threads_mod_time
BEFORE
UPDATE ON threads
FOR EACH ROW EXECUTE PROCEDURE update_modified_column();


CREATE TABLE IF NOT EXISTS posts (post_id SERIAL PRIMARY KEY,
                                                         thread_id INT NOT NULL,
                                                                       user_id INT NOT NULL,
                                                                                   post_content TEXT NOT NULL,
                                                                                                     post_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                          modified_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                   user_ip VARCHAR(255) NOT NULL,
                                  FOREIGN KEY (thread_id) REFERENCES threads(thread_id));


CREATE TRIGGER update_posts_mod_time
BEFORE
UPDATE ON posts
FOR EACH ROW EXECUTE PROCEDURE update_modified_column();


CREATE TABLE IF NOT EXISTS comments (comment_id SERIAL PRIMARY KEY,
                                                               post_id INT NOT NULL,
                                                                           user_id INT NOT NULL,
                                                                                       comment_content TEXT NOT NULL,
                                                                                                            comment_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                    modified_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                             user_ip VARCHAR(255) NOT NULL,
                                     FOREIGN KEY (post_id) REFERENCES posts(post_id));


CREATE TRIGGER update_comments_mod_time
BEFORE
UPDATE ON comments
FOR EACH ROW EXECUTE PROCEDURE update_modified_column();


CREATE TABLE IF NOT EXISTS replies (reply_id SERIAL PRIMARY KEY,
                                                            comment_id INT NOT NULL,
                                                                           parent_reply_id INT, user_id INT NOT NULL,
                                                                                                            reply_content TEXT NOT NULL,
                                                                                                                               reply_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                     modified_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                                                                                                                                                                                              user_ip VARCHAR(255) NOT NULL,
                                    FOREIGN KEY (comment_id) REFERENCES comments(comment_id),
                                    FOREIGN KEY (parent_reply_id) REFERENCES replies(reply_id));


CREATE TRIGGER update_replies_mod_time
BEFORE
UPDATE ON replies
FOR EACH ROW EXECUTE PROCEDURE update_modified_column();

