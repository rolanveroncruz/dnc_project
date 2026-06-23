import { ComponentFixture, TestBed } from '@angular/core/testing';

import { HowToJoinComponent } from './how-to-join-component';

describe('HowToJoinComponent', () => {
  let component: HowToJoinComponent;
  let fixture: ComponentFixture<HowToJoinComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HowToJoinComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(HowToJoinComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
