#include "adbengine.h"
#include <QGuiApplication>
#include <QQmlApplicationEngine>

int
main(int argc, char* argv[])
{
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
  QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
#endif

  QGuiApplication app{ argc, argv };
  // qmlRegisterType注册C++类型至QML
  // arg1:import时模块名
  // arg2:主版本号
  // arg3:次版本号
  // arg4:QML类型名
  qmlRegisterSingletonType<ADBEngine>(
    "ADBEngine.module", 1, 0, "ADBEngine", ADBEngine::adbEngineProvider);

  QQmlApplicationEngine engine{};
  const QUrl url{ QStringLiteral("qrc:/main.qml") };
  QObject::connect(
    &engine,
    &QQmlApplicationEngine::objectCreated,
    &app,
    [url](QObject* obj, const QUrl& objUrl) {
      if (!obj && url == objUrl)
        QCoreApplication::exit(-1);
    },
    Qt::QueuedConnection);
  engine.load(url);
  return app.exec();
}
