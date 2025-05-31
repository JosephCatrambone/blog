---
title: 'BottlePy and GUnicorn'
description: ""
published: 2013-02-13
redirect_from: 
            - https://www.josephcatrambone.com/?p=264
categories: "Rant"
hero: ../../../defaultHero.jpg
---
I had a lot of trouble at work with GUnicorn's workers timing out while debugging BottlePy. There are a slew of arguments to make gunicorn not time out, but because our setup was geared to Heroku, it wasn't clear how to introduce command line arguments. The solution was not to place anything inside the Procfile or to change the calls. The solution was to add the \`options\` parameter into the app.run.Before: app.run(host="0.0.0.0", server='gunicorn', port=os.environ.get('PORT', 8000), debug=settings.DEBUG)After: app.run(host="0.0.0.0", server='gunicorn', port=os.environ.get('PORT', 8000), debug=settings.DEBUG, \*\*serverargs)Where serverargs is a dictionary of the following form:serverargs = {'timeout':'6000', 'workers':'1'} if settings.DEBUG else {}
