## Harraw is a Micro HTTP Load Testing Tools CLI


# Benchmark syntax

We're going to go through all of the benchmark options to understand all
possibilities.

This is a basic benchmark with 2 requests, run 4 plans concurrently against
`http://example.com` servers, executed 5 times. 40 requests in total.

```yaml
base: 'http://example.com'
iterations: 5
rampup: 5

plan:
  - name: Fetch users
    request:
      url: /api/users.json

  - name: Fetch organizations
    request:
      url: /api/organizations
```

### Benchmark main properties

- `base`: Base url for all relative URL's in your plan. (Optional)
- `iterations`: Number of loops is going to do (Optional, default: 1)
- `concurrency`: Number of concurrent iterations. (Optional, default: max)
- `rampup`: Amount of time it will take to start all iterations. (Optional)
- `plan`: List of items to do in your benchmark. (Required)

#### Plan items

- `include`: Include all requests in the given file.
- `request`: Execute a HTTP request.
- `assign`: Assign a value in the context to be interpolated later.

All those three items can be combined with `name` property to be show in logs.

#### Request item properties

- `url`: Url to be request for this item
- `headers`: List of custom headers you want to add in the requests.
- `method`: HTTP method in the requests. Valid methods are GET, POST, PUT, PATCH, HEAD or DELETE. (default: GET)
- `body`: Request body for methods like POST, PUT or PATCH.
- `with_items`: List of items to be interpolated in the given request url.
- `with_items_range`: Generates items from an iterator from start, step (optional, default: 1), stop.
- `with_items_from_csv`: Read the given CSV values and go through all of them as items.
- `shuffle`: Shuffle given items randomly (default: false).
- `pick`: Number of items to pick and perform requests with.
- `assign`: Save the response in the context to be interpolated later.
- `tags`: List of tags for that item.