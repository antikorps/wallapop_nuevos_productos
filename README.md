# Notificador nuevos productos en Wallapop
Programa la ejecución periódica de este script y recibe vía Telegram las notificaciones cuando se incorporen nuevos productos en las URL monitorizadas.

## Instalación y ejecución
El programa no tiene dependencias externas, simplemente debe ejecutarse el binario que se encuentra en la carpeta bin.

Antes de su ejecución debe encontrarse en el mismo directorio que el ejecutable el archivo configuracion.json
```json
{
    "cliente_user_agent": "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:123.0) Gecko/20100101 Firefox/123.0",
    "telegram_bot_token": "bot token proporcionado por @BotFather",
    "telegram_chat": "chat id del grupo al que se espera enviar las notificaciones",
    "telegram_espera": 1,
    "telegram_notificar": true,
    "wallapop_urls": [
        "https://es.wallapop.com/app/search?filters_source=search_box&keywords=port%C3%A1til&longitude=-3.69196&latitude=40.41956",
        "https://es.wallapop.com/app/search?filters_source=search_box&keywords=sobremesa&latitude=40.41956&longitude=-3.69196",
        "https://es.wallapop.com/app/search?filters_source=search_box&keywords=silla%20ordenador&latitude=40.41956&longitude=-3.69196",
        "https://es.wallapop.com/app/search?filters_source=search_box&keywords=microfono%20usb&latitude=40.41956&longitude=-3.69196"
    ],
    "wallapop_espera": 2,
    "wallapop_simultaneidad": 2
}
```
**Importante**: 
- las URL de Wallapop deben empezar siempre por https://es.wallapop.com/app/search?
- Cuando no sea necesario recibir notificaciones en Telegram se puede establecer false a telegram_notificar. Es especialmente útil cuando se ejecuta por primera vez para no recibir notificaciones de todos los productos.
- La espera permite evitar sobrepasar el límite de peticiones permitidas (que generalmente devolverán un status code 429: too many requests)
- La salida informativa es por stdout y los errores por stderr

Después de la ejecución se creará una base de datos llamada **bbdd.sqlite** en el directorio del ejecutable, en ella se guardan todos los ID de los productos notificados para que no vuelvan a notificarse. Puede modificarse (eliminar registros manualmente) o borrar por completo para un reinicio total.