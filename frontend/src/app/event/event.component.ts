import { Component, OnInit, Input } from '@angular/core';
import { Event } from '../models/event';
import { File, EventFile } from '../models/file';

@Component({
  selector: 'app-event',
  templateUrl: './event.component.html',
  styleUrls: ['./event.component.sass']
})
export class EventComponent implements OnInit {

  @Input() event: Event;
  @Input() files: any;

  constructor() { }

  ngOnInit() {
    
  }

}
