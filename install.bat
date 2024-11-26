@echo off

xcopy /s /e nim\target\release\nim.exe "C:\Program Files\NimeScript\nim.exe"

setx PATH "%PATH%;C:\Program Files\NimeScript"

echo NimeScript установлен и доступен в командной строке.