# base image
FROM ghcr.io/astral-sh/uv:python3.13-bookworm

# set environment variables
ENV PYTHONUNBUFFERED=1

# expose port(s)
EXPOSE 8000

# set directory
WORKDIR /opt/app

# add application and install dependencies
COPY pyproject.toml uv.lock .python-version ./
RUN uv sync --frozen
COPY . ./

# run the app
CMD ["uv", "run", "gunicorn", "app:app", "-w", "4", "-b", "0.0.0.0:8000", \
  "--access-logfile", "/dev/stdout", "--error-logfile", "/dev/stderr"]
