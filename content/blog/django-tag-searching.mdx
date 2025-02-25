---
title: 'Django Tag Searching'
description: ""
published: 2012-05-31
redirect_from: 
            - https://www.josephcatrambone.com/?p=149
categories: "Status"
hero: ../../../defaultHero.jpg
---
I had some problems this past week with a pet project.  In Django, I'm writing an image tagging/search social site.  There are pictures, posts, tags and votes.  Users can vote up or down tags.  The problem description is as follows:Select from the set of all pictures the images which do not have a NSFW tag with a sum score of greater than zero.  That is, if an image has a NSFW tag, and the sum of votes on that tag exceeds 0, (+1 is up, -1 is down), then don't display that image.  It was a surprisingly difficult problem which I only solved through the tremendous help of the SomethingAwful forum goons.As Python, this problem is simple:

```
image_set = list();
for image in all_images:
  score = 0;
  for tag in all tags:
    if tag.name == 'nsfw' and tag.target == image.id:
      for vote in votes:
        if vote.target == tag.id:
          score += vote.direction;
  if score > 0:
    image_set.append(image);
```

Now, the above code is a little obtuse because of the way data is stored. We need to track individual votes, rather than the sum of all votes in order to prevent people from casting multiple votes on a single tag. We keep associations between classes rather than just storing arrays of items because this is more conducive to the way the data is stored in the database.Unfortunately, even the slightly wonky code above is not so great, as it requires that we iterate across the database values.I will repeat that.The code above requires that we iterate across the database values. This is bad. Do not iterate across the database items. It's slow, invasive, and defeats the purpose of having a database. Instead, we rely on SQL (or, in this case, the Django Object Relational Mapping) to acquire the data. This offloads the work to the database and allows its finely tuned seek/access/modify algorithms to do their thing without our meddling micromanagement.The final code:

```
latest_pics = Pic.objects.filter(
  id__in=Tag.objects.annotate(
    score=Avg('vote__direction')
  ).exclude(score__lt=0).exclude(name='nsfw')).order_by('-posted')[:desired_results];
```

In parts, Tag.objects.annotate selects tags and sums the scores of their corresponding votes. It then excludes the tags which are confirmed not work safe. After this, we select all Pic objects which have an ID in the set of returned values, sort them, and cut off 10 or 20 results for display on the front page.
