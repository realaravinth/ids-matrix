#!/bin/sh
if [ "$PAM_TYPE" != "open_session" ]
then
  exit 0
else
  /usr/local/bin/ids-matrix $PAM_SERVICE $PAM_USER $PAM_RHOST $PAM_TTY
fi
exit 0
(hellbat)➜  ~ cat $(which notify.sh )
#!/bin/sh
if [ "$PAM_TYPE" != "open_session" ]
then
  exit 0
else
  /usr/local/bin/ids-matrix $PAM_SERVICE $PAM_USER $PAM_RHOST $PAM_TTY
fi
exit 0
