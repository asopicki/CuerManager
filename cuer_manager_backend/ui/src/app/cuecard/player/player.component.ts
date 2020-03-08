import { Component, OnInit, OnDestroy, Input, Output, EventEmitter } from '@angular/core';

import {PitchShifter} from 'soundtouchjs';
import { Cuecard } from 'src/app/events/cuecard';
import { CuecardService } from '../cuecard.service';
import { MessageService } from 'src/app/message.service';

export const enum EventType {
  LoadEvent,
  PlayEvent,
  ProgressEvent,
  StopEvent,
  RecordStartEvent,
  RecordStopEvent,
  SaveEvent
}

export class PlayerEvent {
  type: EventType
  length: number
  progress: number

  constructor(type: EventType) {
    this.type = type;
  }
}

const PLAY_CHAR = "p";
const STOP_CHAR = "s";
const RECORD_CHAR = "r";
const LOAD_CHAR = "l";


@Component({
  selector: 'app-player',
  templateUrl: './player.component.html',
  styleUrls: ['./player.component.scss']
})
export class PlayerComponent implements OnInit, OnDestroy {

  @Input() cuecard: Cuecard;
  @Input() percentage: number;
  @Output() stateChanged = new EventEmitter<PlayerEvent>();
  audioContext;
  audioBuffer;
  audioSource;
  playbackRate = 100;
  playDisabled = true;
  stopDisabled = true;
  loadDisabled = false;
  keyEvents = true;
  eventListener;
  shifter: PitchShifter;

  startTime = 0;
  trackTime = 0;
  length = 0;

  recordCues = false;


  constructor(private service: CuecardService, private messageService: MessageService) {
    this.percentage = 0;
  }

  ngOnInit() {
    this.eventListener = document.addEventListener('keyup', this.keyup.bind(this));
  }

  ngOnDestroy() {
    if (this.eventListener) {
      document.removeEventListener('keyup', this.eventListener);
    }
  }

  setPercentage(percentage: number) {
    this.percentage = percentage;
    this.trackTime = this.length * percentage / 100;
  }

  load(audioFile: Blob) {
    let reader = new FileReader();
    reader.addEventListener('load', this.handleFileLoad.bind(this));
    reader.readAsArrayBuffer(audioFile);
  }

  loadMusicFile() {
    if (this.cuecard && this.cuecard.music_file) {
      this.service.getAudioFile(this.cuecard.music_file).subscribe(
        file => { this.load(file); this.messageService.info("Music file loaded successfully.")},
        (_) => this._logError("Audio file not found!")
      );
    }
  }

  loadExternalFile(e: Event) {
    let target = <HTMLInputElement>e.target;

    if (target.files.length) {
      let file = target.files[0];

      let reader = new FileReader();
      reader.addEventListener('load', this.handleFileLoad.bind(this));

      reader.readAsArrayBuffer(file);
    }
  }

  handleFileLoad(e: ProgressEvent) {
    console.debug('File load complete')
      let arrayBufffer = (<FileReader>e.target).result;
      let audioContext: any = (<any>window).AudioContext || (<any>window).webkitAudioContext;
      let ctx = new audioContext({"latencyHint": "interactive"});

      let bufferSource = ctx.createBufferSource();

      ctx.decodeAudioData(arrayBufffer).then((audioBuffer) => {
        console.debug('Decoded');
        bufferSource.buffer = audioBuffer;
        this.audioContext = ctx;
        this.audioSource = bufferSource;
        this.audioBuffer = audioBuffer;
        this.playDisabled = false;
        this.loadDisabled = true;
        this.length = Math.trunc(audioBuffer.duration);
        let event = new PlayerEvent(EventType.LoadEvent);
        event.length = this.length;
        this.stateChanged.emit(event);
      });
  }

  playRateChange(e: Event) {
    let elem = <HTMLInputElement>e.target;
    this.playbackRate = Number.parseFloat(elem.value);
  }

  changePerc(e: Event) {
    if (this.stopDisabled) {
      const pos = (<HTMLElement>event.target).getBoundingClientRect();
      const relX = (<any>event).pageX - (<any>pos).x;
      const percentage = (relX / (<HTMLElement>event.target).offsetWidth);
      this.percentage = percentage * 100;
      this.trackTime = this.length * percentage;
    }
  }

  resetRate() {
    this.playbackRate = 100;
  }

  play() {
    if (this.audioSource) {
      this.shifter = new PitchShifter(this.audioContext, this.audioBuffer, 16384, this.playerStopped.bind(this));
      this.shifter.tempo = this.playbackRate / 100;
      this.shifter.pitch = 1.0;
      this.playDisabled = true;
      this.stopDisabled = false;
      this.startTime = this.audioContext.currentTime;

      this.shifter.on('play', this.playProgress.bind(this));

      this.shifter.connect(this.audioContext.destination);

      this.shifter.percentagePlayed = this.percentage / 100;
      this.stateChanged.emit(new PlayerEvent(EventType.PlayEvent));
    }
  }

  playerStopped() {
    setTimeout(this.stop.bind(this), 1);
  }

  stop() {
    if (this.shifter) {
      
      this.audioSource = null;
      this.shifter.disconnect();
      this.shifter = null;

      this.audioSource = this.audioContext.createBufferSource();
      this.audioSource.buffer = this.audioBuffer;

      this.playDisabled = false;
      this.stopDisabled = true;
      this.percentage = 0;
      this.trackTime = 0;
      this.stateChanged.emit(new PlayerEvent(EventType.StopEvent));
    }
  }

  save() {
    this.stateChanged.emit(new PlayerEvent(EventType.SaveEvent));
  }

  toggleKeyEvents() {
    this.keyEvents = !this.keyEvents;
  }

  keyup(event: KeyboardEvent) {
    if (!this.keyEvents) {
      return;
    }

    if (!this.playDisabled && event.key === PLAY_CHAR) {
      this.play();
      event.preventDefault();
      return false;
    } else if (!this.stopDisabled && event.key === STOP_CHAR) {
      this.stop();
      event.preventDefault();
      return false;
    } else if (this.stopDisabled && event.key === RECORD_CHAR) {
      this.toggleRecording();
      event.preventDefault();
      return false;
    } else if (event.key == LOAD_CHAR) {
      this.loadMusicFile();
    }
  }

  playProgress(_) {
    if (this.stopDisabled ) {
      return;
    }
    this.trackTime = this.shifter.percentagePlayed / 100 * this.length;
    this.percentage = this.shifter.percentagePlayed;

    let event = new PlayerEvent(EventType.ProgressEvent);
    event.progress = this.percentage;
    this.stateChanged.emit(event);
  }

  toggleRecording() {
    this.recordCues = !this.recordCues;

    let type = (this.recordCues) ? EventType.RecordStartEvent : EventType.RecordStopEvent;
    this.stateChanged.emit(new PlayerEvent(type));
  }

  _logError(message: String) {
    this.messageService.error(`CuecardService: ${message}`);
  }
}
