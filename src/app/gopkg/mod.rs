use actix_web::{HttpRequest, Responder, HttpResponse};

pub async fn index(_req: HttpRequest) -> impl Responder {
	HttpResponse::Ok().content_type("text/html").body("
<html>
<head>
<title>Golang Package</title>
</head>

<body>
	<h1>Golang Package List:</h1>
	<a href=\"https://pkg.agungdp.dev/candi\">Candi</a>
</body>

</html>")
}

pub async fn candi(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body("
<html>
<head>
	<meta name=\"go-import\" content=\"pkg.agungdp.dev/candi git https://github.com/agungdwiprasetyo/candi\">
	<meta name=\"go-source\"
		content=\"pkg.agungdp.dev/candi https://github.com/agungdwiprasetyo/candi https://github.com/agungdwiprasetyo/candi/tree/master{/dir} https://github.com/agungdwiprasetyo/candi/tree/master{/dir}/{file}#L{line}\">
	<meta http-equiv=\"refresh\" content=\"0; url=https://pkg.go.dev/pkg.agungdp.dev/candi\">
</head>

<body>
	Nothing to see here. Please <a href=\"https://pkg.go.dev/pkg.agungdp.dev/candi\">move along</a>.
</body>

</html>")
}
