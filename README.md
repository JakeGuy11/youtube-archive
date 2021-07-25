# Contents
- [Dependencies](#Dependencies)
- [Description](#Description-and-Disclaimer)
- [Contact Me](#Contact-Me)

# Dependencies
1. [ffmpeg](https://ffmpeg.org/ffmpeg.html) version n4.3.1 or higher:
```bash
#Check version
ffmpeg -version
```

2. [youtube-dl](https://github.com/ytdl-org/youtube-dl) version 2021.02.04 or higher:
```bash
#Check version
youtube-dl --version
```

3. [Python](https://www.python.org/) version 3.9.1 or higher:
```bash
#Check version
python --version
```

4. [Requests](https://requests.readthedocs.io/en/master/) version 2.25.1 or higher:
```bash
#Check version
pip show requests
```

# Description and Disclaimer
Youtube-archive is a command line only tool that passively archives YouTube livestreams. It has a customizeable saved and temporary queue that can be different for each user. It is built mainly in C++ but uses a python script for web interactions.\
\
As mentioned in the help page, youtube-archive uses web scraping as an alternative to the YouTube API. This means that it requests the page source from YouTube instead of using the intended YouTube interface. The upside to this (and the reason I chose it) is that it's completely free! The downside is that it breaks often, really whenever YouTube has a major update. Also, it requires extra security so that YouTube doesn't flag your computer as a bot. I've taken most of the usual precautions to ensure this doesn't happen(and it's very nulikely that it will), but still know that there's a chance YouTube might flag your computer. If this app stops working, *please open an error on the github page.* If you're not sure what the problem is, or you suspect that YouTube's flagged your computer as a bot, I'd appreciate it if you attach a copy of the page source of one of the YouTube channels in your queue.

# Contact Me
If you have any feedback, suggestions, errors or just general comments, please email me at Jake_Guy_11@protonmail.ch, or open an error through GitHub.
