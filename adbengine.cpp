#include "adbengine.h"

ADBEngine::ADBEngine(QObject* parent)
  : QObject(parent)
{
  mADBProcess = std::make_shared<QProcess>(new QProcess{});
  qDebug() << DEFAULT_ADB_PATH;
  mADBProcess->setProgram(DEFAULT_ADB_PATH);
  mArgs << ADBORDER::DEVICES;
  mADBProcess->setArguments(mArgs);
  mADBProcess->start();

  mADBProcess->waitForFinished();
  qDebug() << QString::fromLatin1(mADBProcess->readAll());
  QTimer::singleShot(3, [&]() -> void {
    mADBProcess->write("connect 192.168.31.121:5555");

    mADBProcess->waitForFinished();
    qDebug()
      << QString::fromLatin1(mADBProcess->readAll()).replace("\r\n", "\n");
  });

  connect(
    this, &ADBEngine::ADBPathChanged, this, &ADBEngine::onReceivedPathChanged);
}

QObject*
ADBEngine::adbEngineProvider(QQmlEngine* engine, QJSEngine* scriptEngine)
{
  auto adb = new ADBEngine{};
  return adb;
}

void
ADBEngine::sendSignal()
{
  emit ADBPathChanged();
}

QString
ADBEngine::getADBPath() const
{
  return mADBPath;
}

void
ADBEngine::setADBPath(const QString& p)
{
  mADBPath = p;
}

void
ADBEngine::onReceivedPathChanged()
{
  qDebug() << "Changed";
}
