import json
import sqlite3
from datetime import datetime

with open("jcatramb_database_20241107_posts.json", 'rt') as fin:
    data = json.load(fin)

seen_post_ids = dict()
for d in data:
    ts = 0
    try:
        ts = int(datetime.strptime(d['post_modified_gmt'], "%Y-%m-%d %H:%M:%S").timestamp())
    except ValueError:
        pass
    pid = int(d['ID'])
    if pid in seen_post_ids and ts > seen_post_ids[pid]:
        db.execute("DELETE FROM posts WHERE id=?", (pid,))
    db.execute("INSERT INTO posts (id, title, permalink, body_markdown, body_html, tags, date_uploaded) VALUES (?, ?, ?, ?, ?, ?, ?)", (int(d['ID']), d['post_title'], d['guid'], "", d['post_content'], d['post_name'], ts))
    seen_post_ids[pid] = ts