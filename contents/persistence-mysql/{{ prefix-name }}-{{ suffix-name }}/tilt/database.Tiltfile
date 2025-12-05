# -*- mode: Python -*-

# MySQL Database for local development

DB_PORT = {{ database-port }}

# MySQL Docker image
docker_build(
    'database',
    context='.',
    dockerfile_contents='''
FROM mysql:8
ENV MYSQL_USER=test
ENV MYSQL_PASSWORD=test
ENV MYSQL_ROOT_PASSWORD=root
ENV MYSQL_DATABASE=''' + DB_NAME + '''
''',
)

# MySQL Kubernetes deployment
k8s_yaml(blob('''
apiVersion: v1
kind: Service
metadata:
  name: database
  labels:
    app: database
spec:
  ports:
    - port: 3306
      targetPort: 3306
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
        - name: mysql
          image: database
          ports:
            - containerPort: 3306
          env:
            - name: MYSQL_USER
              value: "test"
            - name: MYSQL_PASSWORD
              value: "test"
            - name: MYSQL_ROOT_PASSWORD
              value: "root"
            - name: MYSQL_DATABASE
              value: "''' + DB_NAME + '''"
          readinessProbe:
            exec:
              command:
                - mysqladmin
                - ping
                - -h
                - localhost
                - -u
                - root
                - -proot
            initialDelaySeconds: 10
            periodSeconds: 5
'''))

k8s_resource(
    'database',
    port_forwards=[
        port_forward(DB_PORT, 3306, name='MySQL'),
    ],
)
