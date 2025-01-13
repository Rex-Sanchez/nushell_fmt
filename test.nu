use ~/.cache/starship/init.nu
$env.config.buffer_editor = "nvim"
$env.config.show_banner = false
$env.PROMPT_INDICATOR = "| "


def pm [] {
  mut locations = []

  for x in ["DATA1","DATA2","DATA3"] {
     $locations = $locations | append (glob ~/Mounts/($x)/Movies/**/*.{mp4,mkv,webm} -D | to text)
  }

  let selection = $locations | to text | fzf -i

  if $selection != null {
    mpv $selection
  }

}


def dots [] {
  cd (glob ~/.config/* | to text | fzf -i )
  nvim .
}

def --env p [] {
  let d =  (glob ~/Mounts/DATA3/Projecten/software/**/* --depth 3 | to text | fzf -i )
  if $d != null {
    cd $d
    ls
  }
}

def vpns [] {
  cd (glob ~/Mounts/DATA3/Werk/VPNS/* --no-file -e [.* ] | to text | fzf -i )
  sudo openvpn --config (glob ./**/*.ovpn | to text | fzf -i )
}

def --env etc [] {
  let selection = (glob /etc/* --depth 2 -F | to text | fzf -i )
  if $selection != null {
    cd $selection
    ls
  }

}

def smm [] {
  let url = xclip -o 
  peerflix -f ~/Mounts/DATA0/Movies $url 
}

def rip-audio [url] {
  cd ~/Mounts/DATA2/Music 
  yt-dlp -f 'ba' -x --audio-format mp3 $url
}

def --env cdq [num] {
  try {
    let index = (ls | select $num | get name| get 0)

    let ty = ($index | path type)

    if ($ty == "dir") or ($ty == "symlink") {
      cd $index 
    } else {
      print "Selection is not a directory"
    } 
  } catch {
      print $'index out of bound: [($num)]'
  }
}


def ip [--ext] {
  let local_interfaces = sys net | each { | item | { name: $item.name, mac: $item.mac, ip: ( $item.ip | where { $in.protocol == ipv4 } | get address | get 0) }}

  if $ext {   
    let ip = (http get https://nodedata.io/whoami | scraper -q ".card-header" -s [%inner] | get inner  )  

    return {"external": $ip }
  
  } else {
    return $local_interfaces 
  }
}

def news [] {
  let data = http get https://nos.nl | 
  scraper -q "ul.sc-d9f1a621-11 li a " -s [ href %inner] -a | 
  each {|item| 
    [{
      url: $item.href, 
      title: ($item.inner | scraper -q "h2" -s [%inner] | get inner ), 
      description: ($item.inner | scraper -q "p" -s [%inner] | get inner | $"($in)\n")
    }]} | flatten 
  
  
  let to_select = $data | select title description
  
  print $to_select

  let selection = input "Select Article: "


  try { 
    let len = $data | length
    let selection = $selection | into int

    if $selection < $len {
      let url = $data | get $selection | get url
      let article_url = $"https://nos.nl/($url)"
      let article = http get $article_url 
      let p = {
        title: ($article | scraper -q "h1.sc-3c404ba4-0" -s [%inner] | get inner | to text), 
        text: ($article | scraper -q "p.sc-367cc08e-0" -s [%inner] -a | get inner | to text )
      }
    
      print $p

      let c = input "Do you want to open this article in the browser..? [y/N]: "

      if $c == "y" {
        print 'Opening browser...'
        start $article_url
      }
    } else {
      print 'index out of bound..'
    }
  } catch { 
    print 'Use a index to select the article..'
  }

  



}


def --env mplus_creds [-u username -p password ] {
  export-env { $env.MPLUS_USERNAME = $username }
  export-env { $env.MPLUS_PASSWORD = $password }
}
