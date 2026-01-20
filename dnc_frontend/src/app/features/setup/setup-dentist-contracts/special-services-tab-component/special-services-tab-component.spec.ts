import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SpecialServicesTabComponent } from './special-services-tab-component';

describe('SpecialServicesTabComponent', () => {
  let component: SpecialServicesTabComponent;
  let fixture: ComponentFixture<SpecialServicesTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SpecialServicesTabComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SpecialServicesTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
