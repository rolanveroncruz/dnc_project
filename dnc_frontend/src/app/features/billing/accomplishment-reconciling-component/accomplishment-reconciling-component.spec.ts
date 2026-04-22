import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AccomplishmentReconcilingComponent } from './accomplishment-reconciling-component';

describe('AccomplishmentReconcilingComponent', () => {
  let component: AccomplishmentReconcilingComponent;
  let fixture: ComponentFixture<AccomplishmentReconcilingComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AccomplishmentReconcilingComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AccomplishmentReconcilingComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
