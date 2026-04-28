import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UtilizationReportsComponent } from './utilization-reports-component';

describe('UtilizationReportsComponent', () => {
  let component: UtilizationReportsComponent;
  let fixture: ComponentFixture<UtilizationReportsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UtilizationReportsComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(UtilizationReportsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
