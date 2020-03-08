import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { CuecardCardComponent } from './cuecard-card.component';

describe('CuecardCardComponent', () => {
  let component: CuecardCardComponent;
  let fixture: ComponentFixture<CuecardCardComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ CuecardCardComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(CuecardCardComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
