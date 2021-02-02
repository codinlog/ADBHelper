cmake_minimum_required(VERSION 3.14)

project(QADBHelper LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Quick LinguistTools REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Quick LinguistTools REQUIRED)

set(TS_FILES QADBHelper_zh_CN.ts)

set(QRC
    qml.qrc
    icons.qrc)

set(SOURCE
    main.cpp)

set(PROJECT_SOURCES
       ${SOURCE}
        ${QRC}
        ${TS_FILES}
)


add_executable(QADBHelper
          ${PROJECT_SOURCES}
        )


    qt5_create_translation(QM_FILES ${CMAKE_SOURCE_DIR} ${TS_FILES})


target_compile_definitions(QADBHelper
  PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>)
target_link_libraries(QADBHelper
  PRIVATE Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Quick)
