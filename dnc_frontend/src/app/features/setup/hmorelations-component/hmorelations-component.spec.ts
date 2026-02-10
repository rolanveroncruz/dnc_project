import { ComponentFixture, TestBed } from '@angular/core/testing';

import { HMORelationsComponent } from './hmorelations-component';

describe('HMORelationsComponent', () => {
  let component: HMORelationsComponent;
  let fixture: ComponentFixture<HMORelationsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HMORelationsComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(HMORelationsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
