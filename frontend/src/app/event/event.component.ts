import { Component, OnInit, Input } from '@angular/core';
import { Event } from '../models/event';

@Component({
  selector: 'app-event',
  templateUrl: './event.component.html',
  styleUrls: ['./event.component.sass']
})
export class EventComponent implements OnInit {

  @Input() event: Event;
  @Input() files: any;
  public isCollapsed = true;

  constructor() { }

  ngOnInit() {
    
  }

}
