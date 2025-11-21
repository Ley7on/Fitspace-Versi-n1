Fitspace - Plataforma de acompañamiento y gestión de gimnasios

Fitspace es una plataforma web integral de gestión para gimnasios, diseñada con un enfoque modular y escalable. Desarrollada en Django y con base de datos MySQL, permite la administración eficiente de clientes, entrenadores, rutinas, pagos y más, mejorando la experiencia del usuario a través de un sistema eficiente de seguimiento de progreso, reservas y control de asistencia.

Estructura del Repositorio

Este repositorio contiene la siguiente estructura de directorios:

Fitspace/
│
├── backend/                 # Código fuente del backend (Django)
│   ├── models.py            # Modelos de base de datos
│   ├── views.py             # Vistas de la aplicación
│   ├── urls.py              # Rutas del proyecto
│   └── ...                  # Otros archivos de configuración del backend
│
├── frontend/                # Código fuente del frontend
│   ├── templates/           # Plantillas HTML de la interfaz de usuario
│   ├── static/              # Archivos estáticos (CSS, JS, imágenes)
│   └── ...                  # Otros archivos de configuración del frontend
│
├── scripts/                 # Scripts útiles para el proyecto
│   ├── deploy.sh            # Script de despliegue
│   ├── setup.sh             # Script de configuración del entorno
│   └── ...                  # Otros scripts del proyecto
│
├── config.env               # Variables de configuración del entorno
├── requirements.txt         # Dependencias de Python
├── README.md                # Documentación mínima del repositorio
└── manage.py                # Archivo principal para ejecutar el servidor y migraciones

Documentación mínima
1. Código fuente y scripts
Backend

El backend está desarrollado en Django, un framework Python para el desarrollo web. Los archivos clave son:

models.py: Define los modelos de base de datos utilizados para almacenar información sobre los usuarios, rutinas y demás.

views.py: Contiene la lógica para manejar las solicitudes HTTP y devolver respuestas, interactuando con los modelos de la base de datos.

urls.py: Define las rutas de la aplicación, asociando las vistas a las URLs correspondientes.

Frontend

El frontend está compuesto por plantillas HTML con lógica dinámica proporcionada por Django Templates y archivos estáticos como CSS y JavaScript:

templates/: Contiene las plantillas HTML que definen la estructura de las páginas web.

static/: Contiene archivos CSS y JS para estilizar y añadir interactividad a la interfaz de usuario.

Scripts

Los scripts proporcionan funciones útiles para la configuración y despliegue del proyecto:

deploy.sh: Script para automatizar el despliegue del proyecto en un servidor.

setup.sh: Script para la instalación de dependencias y la configuración inicial del entorno de desarrollo.

2. Configuraciones

Este proyecto requiere la configuración de un entorno de desarrollo y un entorno de producción. Los archivos principales de configuración incluyen:

config.env: Contiene variables de configuración como las credenciales de la base de datos y claves de API necesarias para ejecutar la aplicación en diferentes entornos.

3. Instalación y uso

Clona el repositorio:

git clone https://github.com/Ley7on/Fitspace-Versi-n1.git
cd Fitspace-Versi-n1


Configura el entorno virtual:

python -m venv venv
source venv/bin/activate  # En Linux/macOS
venv\Scripts\activate     # En Windows


Instala las dependencias:

pip install -r requirements.txt


Configura el entorno:

Asegúrate de crear el archivo config.env con las variables necesarias (por ejemplo, claves de base de datos, etc.).

Realiza las migraciones de la base de datos:

python manage.py migrate


Crea un superusuario para acceder al panel de administración de Django:

python manage.py createsuperuser


Ejecuta el servidor:

python manage.py runserver
