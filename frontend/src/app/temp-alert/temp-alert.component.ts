import { Component, OnInit, Input } from '@angular/core';
import { Subject } from 'rxjs';
import { debounceTime } from 'rxjs/operators';

@Component({
  selector: 'app-temp-alert',
  templateUrl: './temp-alert.component.html',
  styleUrls: ['./temp-alert.component.sass']
})
export class TempAlertComponent implements OnInit {

  @Input() alert_type: string;
  @Input() changeMessage = (message: string) => {
    this.alert_subject.next(message);
  };

  private alert_message: string;
  private alert_subject = new Subject<string>();

  constructor() { }

  ngOnInit() {
    this.alert_subject.subscribe((message) => this.alert_message = message);
    this.alert_subject.pipe(
      debounceTime(5000)
    ).subscribe(() => this.alert_message = null);
  }

  /*public changeMessage(message: string) {
    this.alert_subject.next(message);
  }*/

}
