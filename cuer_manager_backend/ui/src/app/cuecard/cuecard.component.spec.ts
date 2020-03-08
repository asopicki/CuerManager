import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { CuecardComponent } from './cuecard.component';

describe('CuecardComponent', () => {
  let component: CuecardComponent;
  let fixture: ComponentFixture<CuecardComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ CuecardComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(CuecardComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
