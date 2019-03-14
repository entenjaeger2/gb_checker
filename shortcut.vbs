Set wshShell = CreateObject("WScript.Shell")
Set fso = CreateObject("Scripting.FileSystemObject")
Set args = WScript.Arguments

If args.Count < 1 Then
WScript.Echo "Dateiname fehlt."
WScript.Quit
End If

ziel = args(0)
ziel = fso.GetAbsolutePathName(ziel)
idxName = InStrRev(ziel, "\")
idxExt = InStrRev(ziel, ".")
If idxExt < idxName Then idxExt = Len(ziel) + 1
ordner = Left(ziel, idxName - 1)
linkName = Mid(ziel, idxName + 1, idxExt - idxName - 1)
desktop = wshShell.SpecialFolders("Startup")

Set link = wshShell.CreateShortcut(desktop & "\" & linkName & ".lnk")
link.TargetPath = ziel
link.WorkingDirectory = ordner
link.Save