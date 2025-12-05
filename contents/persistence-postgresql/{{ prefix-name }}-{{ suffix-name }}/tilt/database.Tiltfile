# -*- mode: Python -*-

# PostgreSQL Database for local development

DB_PORT = {{ database-port }}

# PostgreSQL Docker image
docker_build(
    'database',
    context='.',
    dockerfile_contents='''
FROM postgres:15-alpine
ENV POSTGRES_USER=test
ENV POSTGRES_PASSWORD=test
ENV POSTGRES_DB=''' + DB_NAME + '''
''',
)

# PostgreSQL Kubernetes deployment
k8s_yaml(blob('''
apiVersion: v1
kind: Service
metadata:
  name: database
  labels:
    app: database
spec:
  ports:
    - port: 5432
      targetPort: 5432
  selector:
    app: database
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: database
  labels:
    app: database
spec:
  replicas: 1
  selector:
    matchLabels:
      app: database
  template:
    metadata:
      labels:
        app: database
    spec:
      containers:
        - name: postgres
          image: database
          ports:
            - containerPort: 5432
          env:
            - name: POSTGRES_USER
              value: "test"
            - name: POSTGRES_PASSWORD
              value: "test"
            - name: POSTGRES_DB
              value: "''' + DB_NAME + '''"
          readinessProbe:
            exec:
              command:
                - pg_isready
                - -U
                - test
                - -d
                - ''' + DB_NAME + '''
            initialDelaySeconds: 5
            periodSeconds: 5
'''))

k8s_resource(
    'database',
    port_forwards=[
        port_forward(DB_PORT, 5432, name='PostgreSQL'),
    ],
)
