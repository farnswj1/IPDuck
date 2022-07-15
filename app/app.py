from flask import Flask, request, render_template
from werkzeug.middleware.proxy_fix import ProxyFix

app = Flask(__name__)
app.wsgi_app = ProxyFix(
    app.wsgi_app,
    x_for=1,
    x_proto=1,
    x_host=1,
    x_port=1,
    x_prefix=1
)


@app.route('/', methods=['GET'])
def get_client_ip():
    client_ip = request.environ.get('HTTP_X_FORWARDED_FOR', request.remote_addr)
    remote_port = request.environ.get(
        'HTTP_X_FORWARDED_PORT',
        request.environ.get('REMOTE_PORT', 'N/A')
    )
    browser = request.environ.get('HTTP_USER_AGENT', 'N/A')

    context = {
        'client_ip': client_ip,
        'remote_port': remote_port,
        'browser': browser
    }

    return render_template('index.html', **context)
