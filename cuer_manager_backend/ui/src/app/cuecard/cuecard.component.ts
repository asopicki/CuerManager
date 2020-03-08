import { Component, OnInit, OnDestroy, AfterViewInit, ViewChild, ChangeDetectorRef } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

import { MatDialog } from '@angular/material/dialog';

import { CuecardService } from './cuecard.service';
import { PlayerEvent, EventType, PlayerComponent } from './player/player.component';

import { Cuecard } from '../events/cuecard';
import { MarkData } from './markdata';
import { MessageService } from '../message.service';
import { Subscription } from 'rxjs';

import * as marked from 'marked';

const CUE_CHAR = "c";

@Component({
  selector: 'app-cuecard',
  templateUrl: './cuecard.component.html',
  styleUrls: ['./cuecard.component.scss']
})
export class CuecardComponent implements OnInit, OnDestroy, AfterViewInit {

  @ViewChild(PlayerComponent, {static: true})
  private playerComponent: PlayerComponent;
  private refreshSubject: Subscription;
  cuecard: Cuecard;
  uuid: String;
  playing = false;
  recording = false;
  perc = 0;
  length: number;
  eventListener;
  
  marks: String[] = [];
  headlines: String[] = [];
  karaokeMarks = [];

  //editing
  saveButtonClasses = ['hidden'];
  editButtonClasses = [];
  playerClasses = [];
  markdownClasses = ['hidden'];
  cuecardClasses = [];
  editable = 'false';
  debounceTime: number;
  lastUpdate: number;
  timer;

  currentH: HTMLElement;
  currentP: HTMLElement;
  currentElement = 0;
  currentIndex = 0;
  marked;
  
  constructor(
    private route: ActivatedRoute, 
    private service: CuecardService, 
    private changeDetection: ChangeDetectorRef,
    private messageService: MessageService,
    private dialog: MatDialog )  { 
    
  }

  ngOnInit() {
    this.route.paramMap.subscribe(params => {
      this.uuid = params.get('uuid');

      if (this.uuid) {
        this.loadCuecard();

        this.loadCuecardContent();
      }
    });
  
    this.eventListener = document.addEventListener('keyup', this.keyup.bind(this));
    this.refreshSubject = this.service.$libraryRefreshed.subscribe( update => {
      this.loadCuecard();
      this.loadCuecardContent();
    });
  }

  ngAfterViewInit() {

  }

  ngOnDestroy() {
    this.refreshSubject.unsubscribe();

    if (this.eventListener) {
      document.removeEventListener('keyup', this.eventListener);
    }
  }

  edit(cuecard: Cuecard) {
    this.editable = 'true';
    this.editButtonClasses=['hidden'];
    this.saveButtonClasses=[];
    this.playerClasses=['hidden'];
    this.markdownClasses=['visible'];
    //this.cuecardClasses=['hidden'];
    this.lastUpdate = Date.now();
    this.debounceTime = Date.now();

    this.playerComponent.toggleKeyEvents();

    return false;
  }

  parse(_event: Event, elem: HTMLElement) {
    let time = Date.now();
    if ((time - this.debounceTime < 300) && (time - this.lastUpdate < 2000)) {
      if (this.timer) {
        clearTimeout(this.timer);
      }
      this.timer = setTimeout(this.updatePreview, 500, elem);
      return;
    }
    clearTimeout(this.timer);
    this.timer = null;
    this.updatePreview(elem);
  }

  private updatePreview(elem: HTMLElement) {
    let time = Date.now();
    this.debounceTime = time;
    this.lastUpdate = time;
    let markdown = marked(elem.innerText);

    document.getElementById('cuecard').innerHTML = markdown;
  }

  saveCuecard(cuecard: Cuecard) {
    this.editable = 'false';
    this.editButtonClasses=[];
    this.saveButtonClasses=['hidden'];
    this.playerClasses=[];
    this.markdownClasses=['hidden'];
    //this.cuecardClasses=[];

    this.playerComponent.toggleKeyEvents();

    let content = document.getElementById('markdown').innerText;

    if (cuecard.content != content) {
      this.service.updateContent(cuecard.uuid, content).subscribe(_ => {
        this.loadCuecardContent();
        cuecard.content = content;
      })
    }
    
    
    return false;
  }

  private loadCuecard() {
    this.service.getCuecard(this.uuid).subscribe(cuecard => {
      this.cuecard = cuecard
      document.title = this.cuecard.title.toString() + ' - Cuer Event Manager';
      if (this.cuecard.karaoke_marks) {
        let marks = JSON.parse(this.cuecard.karaoke_marks.toString());

        if (Array.isArray(marks)) {
          this.marks = marks;
        } else if (typeof(marks) === 'object') {
          marks = new MarkData(marks);
          this.marks = marks.marks;
          this.headlines = marks.headlines;
        }
      }

      document.getElementById('markdown').innerText = cuecard.content.toString();
    }, (_) => {
      this._logError("Unable to load cuecard!");
    });
  }

  private loadCuecardContent() {
    this.service.getCuecardContent(this.uuid).subscribe(content => {
      this.updateContent(content);
    }, (_) => {
      this._logError("Unable to load cuecard content.");
    });
  }

  private updateContent(content: String) {
    document.getElementById('cuecard').innerHTML = content.toString();

    let headlines = document.querySelectorAll('#cuecard > h1');

    headlines.forEach(headline => headline.addEventListener('dblclick', this.startAtHeadline.bind(this)));
  }

  onPlayerStateChanged(event: PlayerEvent) {
    switch(event.type) {
      case EventType.LoadEvent: {
        this.length = event.length;
        break;
      }
      case EventType.PlayEvent: {
        this.playing = true;

        if (this.recording) {
          this.marks = [];
          this.headlines = [];
          document.getElementById('cuecard').focus();
        } else {
          this.karaokeMarks = this.marks.slice();
  
          if (this.perc > 0) {
            while (this.karaokeMarks.length) {
              let markTime = this.length * this.perc / 100;
              if (markTime > Number.parseFloat(this.karaokeMarks[0])) {
                this.karaokeMarks.shift();
                this.highlight();
              } else {
                break;
              }
            }
          }
        }
        break;
      }
      case EventType.StopEvent: {
        this.playing = false;
        if (this.recording) {
          console.log("Marks:", this.marks);
          console.log("Headlines:", this.headlines);
        }

        if (this.currentP) {
          let elem = document.createElement('p');
          elem.innerHTML = this.currentP.innerText;
          this.currentP.replaceWith(elem);
        }

        this.currentP = null;
        this.currentH = null;
        this.currentIndex = 0;
        this.currentElement = 0;
        this.perc = 0;
        this.changeDetection.detectChanges();
        console.log('stop');

        this.service.cued_at(this.cuecard).subscribe(()=> {});
        break;
      }
      case EventType.RecordStartEvent: {
        this.recording = true;
        break;
      }
      case EventType.RecordStopEvent: {
        this.recording = false;
        break;
      }
      case EventType.SaveEvent: {
        this.save();
        break;
      }
      case EventType.ProgressEvent: {
        this.perc = event.progress;
        
        if (!this.recording && this.karaokeMarks) {
          this.karaoke();
        }
        break;
      }
    }
  }

  keyup(event) {
    if (this.recording && event.key === CUE_CHAR) {
      this.highlight();

      let markTime = this.length * this.perc / 100;
      this.marks.push(markTime.toFixed(5));

      let headline = this.getCurrentHeadline();
      if (this.currentH == null) {
        this.headlines.push('0.0');
      } else if (this.currentH != headline) {
        this.headlines.push(markTime.toFixed(5));
      }
      this.currentH = headline;
      event.preventDefault();
      return false;
    }
  }

  startAtHeadline(e: Event) {
    if (!this.headlines) {
      return true;
    }

    if (this.playing) {
      return true;
    }

    let headline = <HTMLElement>e.target;
    let headlines = document.querySelectorAll('#cuecard > h1');
    
    let i = 1;

    while (i < headlines.length) {
      let current = headlines.item(i)
      if (current == headline) {
        if (i-1 < this.headlines.length) {
          let markTime = Number.parseFloat(this.headlines[i-1].toString());
          let perc = markTime / this.length * 100 - 0.5;

          console.log(markTime, perc);
          if (perc > 0 ) {
            this.perc = perc;
          } else {
            this.perc = 0;
          }
          this.playerComponent.setPercentage(this.perc);
        }
      }
      i++;
    }

    e.preventDefault();
    return false;
  }

  save() {
    let data = new MarkData({marks: this.marks, headlines: this.headlines});
    this.service.setMarks(this.uuid, data).subscribe(
      () => this.messageService.info("Cues have been saved."), 
      (_) => {
        this._logError("Saving marks failed!");
    });
    return false;
  }

  karaoke() {
    if (this.karaokeMarks.length) {
     let markTime = this.length * this.perc / 100;
     if (markTime > Number.parseFloat(this.karaokeMarks[0])-0.1) {
       this.karaokeMarks.shift();
       this.highlight();
     }
    }
    
  }

  highlight() {
    const style = "background-color: #232F34; color: white; font-weight: bold; font-size: 1.2em; padding: 0.2em 0.3em;"
    if (this.currentP) {
      let text = this.currentP.innerText;
      let parts = text.split(';');

      while (true) {
        if (this.currentIndex >= parts.length) {
          this.currentP.innerHTML = parts.join(';'); //remove highlight from current line
          this.updateCurrentParagraph();
          if (!this.currentP) {
            return;
          }
          //reset state
          this.currentIndex = 0;
          text = (<HTMLElement>this.currentP).innerText;
          parts = text.split(';');
          continue;
        }
        if (parts[this.currentIndex].trim() === '') {
          this.currentIndex++;
          continue;
        }

        let cueterm = parts[this.currentIndex];

        //cueterm = '<span style="' + style + '">' + cueterm + '</span> ';
        cueterm = '<span class="highlight">' + cueterm + '</span> ';

        parts[this.currentIndex++] = cueterm;

        let elem = document.createElement('p');
        elem.innerHTML = parts.join(';');
        this.currentP.replaceWith(elem);
        this.currentP = <HTMLElement>elem;
        this.scrollTo(elem);
        break;
      }
    } else {
      //get first paragraph of the cuecard
      this.updateCurrentParagraph();

      if (!this.currentP) {
        return;
      }

      //hightlight first cue term if any
      let text = (<HTMLElement>this.currentP).innerText;

      let parts = text.split(';');

      if (parts.length == 0) {
        return; //return if there are no parts
      }

      let cueterm = parts[this.currentIndex++];

      cueterm = '<span class="highlight">' + cueterm + '</span>;';

      parts.shift();

      let elem = document.createElement('p');
      elem.innerHTML = cueterm + parts.join(';');
      this.currentP.replaceWith(elem);
      this.currentP = elem;
      this.scrollTo(elem);
    }


    
  }

  updateCurrentParagraph() {
    let nodes = document.querySelectorAll('#cuecard blockquote p');

    if (this.currentElement >= nodes.length) {
      this.currentP = null;
      return;
    }

    this.currentP = <HTMLElement>nodes.item(this.currentElement++);
  }

  getCurrentHeadline(): HTMLElement | null {
    let current = <HTMLElement>this.currentP.parentNode;

    while (current.previousSibling) {
      if (current.previousSibling.nodeName.toLowerCase() === 'h1') {
        return <HTMLElement>current.previousSibling;
      }

      current = <HTMLElement>current.previousSibling;
    }

    return null;
  }

  scrollTo(elem: HTMLElement) {
    if ((<any>elem).scrollIntoViewIfNeeded) {
      (<any>elem).scrollIntoViewIfNeeded(true);
    } else {
      elem.scrollIntoView({"behavior": "auto", "block": "center", "inline": "center"});
    }
  }

  _logError(message: String) {
    this.messageService.error(`CuecardService: ${message}`);
  }
}
