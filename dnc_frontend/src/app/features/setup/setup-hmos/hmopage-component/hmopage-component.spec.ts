import { ComponentFixture, TestBed } from '@angular/core/testing';

import { HMOPageComponent } from './hmopage-component';

describe('HMOPageComponent', () => {
  let component: HMOPageComponent;
  let fixture: ComponentFixture<HMOPageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HMOPageComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(HMOPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
