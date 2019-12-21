import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { TempAlertComponent } from './temp-alert.component';

describe('TempAlertComponent', () => {
  let component: TempAlertComponent;
  let fixture: ComponentFixture<TempAlertComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ TempAlertComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(TempAlertComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
