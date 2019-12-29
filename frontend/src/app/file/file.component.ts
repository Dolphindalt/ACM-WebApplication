import { Component, OnInit, Input } from '@angular/core';
import { Inject } from '@angular/core';

@Component({
  selector: 'app-file',
  templateUrl: './file.component.html',
  styleUrls: ['./file.component.sass']
})
export class FileComponent implements OnInit {

  @Input() file: File;
  private backend_url: String;

  constructor(
    @Inject('BASE_API_URL') private baseUrl: string
  ) {
    this.backend_url = baseUrl;
  }

  ngOnInit() {
  }

}
