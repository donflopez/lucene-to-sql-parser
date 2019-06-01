# 💱 LuceneQS To SQL

[![Build Status](https://travis-ci.org/donflopez/lucene-to-sql-parser.svg?branch=master)](https://travis-ci.org/donflopez/lucene-to-sql-parser)

This simple library is basically a grammar for [lalrpop](https://github.com/lalrpop/lalrpop) that transforms a query in the lucene query syntax to a sql where clause.

---

## Examples:

```sql
---Lucene---
field:(a OR b)
---SQL---
field = 'a' OR field = 'b'

---Lucene---
a:(1 OR c*)
---SQL---
field = 1 OR field like 'c%'

---Lucene---
((ee:\"val\" OR ee:*toma) AND NOT p:[* TO pepe]) AND a:(what OR no AND >=2) OR c:(>=awesome OR <excellent)
---SQL---
((ee = 'val' OR ee like '%toma') AND NOT p <= 'pepe') AND (a = 'what' OR a = 'no' AND a >= 2) OR (c >= 'awesome' OR c < 'excellent')
```

## Getting Started

TODO

## ❗️Limitations

### Bare queries

LuceneQS can get single values without specifying any field, sql engines no, this is a limitation on the query parsing, to solve this I'm building an API where you can set the fields you want to lookup if a raw value is set so it will treat them like:

```sql
---Lucene---
myvalue
---SQL Field specidied [name, description, body, summary]---
name = 'myvalue' OR description = 'myvalue' OR body = 'myvalue' OR summary = 'myvalue'
```

### Token Search

You probably noticed that when you use `term:value` the translation is `term = 'value'` but in LQS would like `term like '%value%'`. I'm planning on changing it or leaving that configurable, I'd like to restrict that for perf reasons in some of my proyect but then the syntax translation is not correct and would make others not use it.

### 🚫Not supported features (yet)

There is a few things not supported that are going to be, but there are others that won't.

- [All boolean operators (will be supported)](https://www.elastic.co/guide/en/elasticsearch/reference/7.1/query-dsl-query-string-query.html#_boolean_operators)
- [Boosting](https://www.elastic.co/guide/en/elasticsearch/reference/7.1/query-dsl-query-string-query.html#_boosting)
- [Proximity](https://www.elastic.co/guide/en/elasticsearch/reference/7.1/query-dsl-query-string-query.html#_proximity_searches)
- [Fuzziness](https://www.elastic.co/guide/en/elasticsearch/reference/7.1/query-dsl-query-string-query.html#_fuzziness)
