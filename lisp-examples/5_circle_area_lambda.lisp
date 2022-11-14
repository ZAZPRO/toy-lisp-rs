(
    (define pi 314)
    (define r 10)
    (define sqr (lambda (r) (* r r)))
    (define area (lambda (r) (* pi (sqr r))))
    (area r)
)