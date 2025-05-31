def update_page_text(text) -> str:
    if text.strip().startswith("---"):
        start = text.find("---\n")
        end = text.find("---\n", start+1)
        header = text[start:end+len("---")]
        fields = header.split("\n")
        fields = [f.split(":") for f in fields]
        # Everything is separated by colons.
    elif text.strip().startswith("+++"):
        start = text.find("+++\n")
        end = text.find("+++\n", start+1)
        header = text[start:end+len("+++")]
        fields = header.split("\n")
        fields = [f.split("=") for f in fields]
    #print(header)
    #print(fields)
    title = ""
    date = ""
    tags = ""
    for f in fields:
        if f[0].lower().strip() == "title":
            title = f[1].strip().strip("'").strip('"')
        elif f[0].lower().strip() == "published":
            date = f[1].strip().strip("'").strip('"')
        elif f[0].lower().strip() == "date":
            date = f[1].strip().strip('"').strip("'")
        elif f[0].lower().strip() == "categories":
            tags = f[1].strip().strip('"').strip("'").split()
            tags = ",".join([f"\"{t}\"" for t in tags])
    new_header = f"""+++
title = "{title}"
date = "{date}"

[taxonomies]
tags=[{tags}]
+++\n"""
    return new_header + text[end+len("---"):]
