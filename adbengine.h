#ifndef ADBENGINE_H
#define ADBENGINE_H
#include <QDebug>
#include <QFile>
#include <QObject>
#include <QProcess>
#include <QProcessEnvironment>
#include <QQmlEngine>
#include <QTimer>
#include <memory>

const static QString DEFAULT_ADB_PATH{ "./adb/adb" };

namespace ADBORDER {
const QString VERSION{ "version" };
const QString DEVICES{ "devices" };
}

class ADBEngine : public QObject
{
  Q_OBJECT

  Q_PROPERTY(
    QString mADBPath READ getADBPath WRITE setADBPath NOTIFY ADBPathChanged);

private:
  std::shared_ptr<QProcess> mADBProcess;
  QStringList mArgs;
  QString mADBPath{ "./adb/adb" };

public:
  ADBEngine(QObject* parent = nullptr);
  static QObject* adbEngineProvider(QQmlEngine* engine,
                                    QJSEngine* scriptEngine);
  //通过Q_INVOKABLE宏标记的public函数可以在QML中访问
  Q_INVOKABLE void sendSignal(); //功能为发送信号
  QString getADBPath() const;
  void setADBPath(const QString&);
Q_SIGNALS:
  void ADBPathChanged();

public Q_SLOTS:
  void onReceivedPathChanged();
};

#endif // ADBENGINE_H
